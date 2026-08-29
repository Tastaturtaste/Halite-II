import subprocess
import json

try:
    with open("config.json") as _config_fh:
        _config_data = json.load(_config_fh)
        UNSANDBOXED = bool(_config_data.get("UNSANDBOXED", False))
        HEALTH_CHECK_PORT = int(_config_data.get("HEALTH_CHECK_PORT", 5002))
except Exception:
    UNSANDBOXED = False
    HEALTH_CHECK_PORT = 5002


def kill_processes_as(user, process_name=None):
    """
    Kill all processes of a given name belonging to a given user.
    :param user:
    :param process_name: Name of process to kill, if not specified or None will
                         kill all of the user's processes.
    :return:
    """
    if UNSANDBOXED:
        return
    cmd_args = ["sudo", "-H", "-u", user, "-s",
                "killall", "-9", "-u", user]
    if process_name is not None:
        cmd_args.append(process_name)
    subprocess.call(cmd_args,
                    stderr=subprocess.PIPE,
                    stdout=subprocess.PIPE)
