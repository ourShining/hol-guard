use super::{interpreter_options, names, program};

fn words(text: &str) -> Vec<String> {
    text.split(' ').map(str::to_owned).collect()
}

#[test]
fn programs_that_only_reshape_their_input_pass() {
    for source in [
        "import json,sys; d=json.load(sys.stdin); print(d['state'])",
        "import sys,json; d=json.load(sys.stdin); print(json.dumps({k:d.get(k) for k in ('a','b')}, indent=2))",
        "\nimport json,sys\nfor c in json.load(sys.stdin):\n    if c['state'] not in ('SUCCESS','SKIPPED'): print(c['state'], c['name'], c['link'])",
        "import json,sys; rs=json.load(sys.stdin); [print(r.get('id'), str(r.get('command',''))[:70].replace(chr(10),' ')) for r in rs]",
        "import json, sys\nfrom collections import Counter\nprint(Counter(x['kind'] for x in json.load(sys.stdin)).most_common(5))",
        "import json,sys\nprint(f\"{len(json.load(sys.stdin))} rows, first {sys.argv[1]!r:>10}\")",
        "import re,sys\nfor line in sys.stdin:\n    m = re.compile(r'(\\d+)').search(line)\n    if m: print(m.group(1), file=sys.stderr)",
        "from json import (\n    load,\n    dumps,\n)\nimport sys\nprint(dumps(load(sys.stdin)))",
        "import json,sys\ndef walk(o):\n    if isinstance(o, dict):\n        for k, v in o.items(): walk(v)\nwalk(json.load(sys.stdin))",
        "import json,sys; print(f'{\"x\" if 1 else \"y\"}')",
    ] {
        assert!(program::screened(source), "{source}");
    }
}

#[test]
fn programs_that_reach_past_their_input_fail() {
    for source in [
        "import os; print(os.listdir('.'))",
        "import subprocess; subprocess.run(['ls'])",
        "import json as j; print(j)",
        "import os.path",
        "from os import system",
        "from json import *",
        "from . import x",
        "import json; print(json.decoder)",
        "import sys; print(sys.modules)",
        "import sys; m = sys; print(m)",
        "print(open('/etc/hosts').read())",
        "print(eval('1+1'))",
        "exec('print(1)')",
        "print(getattr(print, 'x'))",
        "print(globals())",
        "print(().__class__)",
        "g = (x for x in []); print(g.gi_frame.f_globals)",
        "import sys; print(sys._getframe())",
        "x = input()",
        "help(print)",
        "breakpoint()",
        "print(type(1))",
        "print(f'{open(\"x\")}')",
        "print(f\"{f'{open(1)}'}\")",
        "print(f\"{x[\"k\"]}\")",
        "print(f'{x\\n}')",
        "print('unterminated)",
        "# -*- coding: rot13 -*-\nprint(1)",
        "print('caf\u{e9}')",
        "import shlex; s = shlex.shlex('x'); s.source = 'source'",
        "import re; c = compile('1', 'x', 'eval')",
        "x = 1 \\ 2",
        "if True: import os",
        "import\njson",
    ] {
        assert!(!program::screened(source), "{source}");
    }
}

#[test]
fn only_quiet_interpreter_switches_lead_to_the_program() {
    for (arguments, isolated, program) in [
        ("-c X", false, 1),
        ("-I -c X", true, 2),
        ("-Ic X arg", true, 1),
        ("-u -B -c X", false, 3),
    ] {
        let options = interpreter_options(&words(arguments)).expect(arguments);
        assert_eq!(options.ignores_environment, isolated, "{arguments}");
        assert_eq!(options.safe_path, isolated, "{arguments}");
        assert_eq!(options.program, program, "{arguments}");
    }
    for arguments in [
        "-c",
        "script.py",
        "-m json.tool",
        "-i -c X",
        "-X dev -c X",
        "-cX",
        "--c X",
        "-W error -c X",
    ] {
        assert!(
            interpreter_options(&words(arguments)).is_none(),
            "{arguments}"
        );
    }
}

#[test]
fn only_variables_that_steer_the_interpreter_count() {
    for name in [
        "PYTHONPATH",
        "PYTHONSTARTUP",
        "PYTHONHOME",
        "pythonpath",
        "PYTHONWARNINGS",
        "PYTHONUSERBASE",
    ] {
        assert!(names::steering_variable(name), "{name}");
    }
    for name in [
        "PYTHONUNBUFFERED",
        "PYTHONIOENCODING",
        "PYTHONDONTWRITEBYTECODE",
        "PATH",
        "PY",
        "\u{e9}PYTHON",
    ] {
        assert!(!names::steering_variable(name), "{name}");
    }
}

#[test]
fn standard_library_names_shadow_whatever_their_case() {
    for stem in ["json", "JSON", "traceback", "_colorize", "sitecustomize"] {
        assert!(names::shadows_standard_library(stem), "{stem}");
    }
    for stem in ["pkg", "scripts", "", "tests", "README"] {
        assert!(!names::shadows_standard_library(stem), "{stem}");
    }
}
