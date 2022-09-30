# This is only needed to run "maturin develop" as "Before launch" action in PyCharm
# Using maturin develop does not pick up the virtual env

import subprocess

cmd = subprocess.run(["maturin", "develop"])
exit(cmd.returncode)
