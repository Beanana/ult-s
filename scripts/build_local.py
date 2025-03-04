import os
import shutil
from zipfile import ZipFile

stream = os.popen('cargo skyline build')
output = stream.read()
output
os.chdir('../')
print(os.getcwd())
old = r"target\aarch64-skyline-switch\debug\libplugin.nro"
new = r"releases"
old_rename = r"libplugin.nro"
rename = r"plugin.nro"
file_name = "plugin.zip"
 
if os.path.exists(r'target'):
    os.chdir(r'target')
    print(os.listdir())
    if os.path.exists(r'aarch64-skyline-switch'):
        os.chdir(r'aarch64-skyline-switch')
        print(os.listdir())
        if os.path.exists(r'debug'):
            os.chdir(r'debug')
            print(os.listdir())
            old = os.path.join(os.path.abspath(os.getcwd()), r'libplugin.nro')
            os.chdir('../')
            os.chdir('../')
            os.chdir('../')
            print(os.getcwd())
            if os.path.exists(new) == False:
                os.mkdir(new)
            print(old)
            shutil.move(old, new)
            print(os.listdir())
            shutil.move(os.path.join(new, r'libplugin.nro'), os.path.join(new, r'plugin.nro'))
            print("done")
        else:
            print('aarch64-skyline-switch does not exist')
    else:
        print('aarch64-skyline-switch does not exist')
else:
    print('target does not exist')

"""
if os.path.exists(old) == False:
    print("issue")

if os.path.exists(new) == False:
    os.mkdir(new)

shutil.move(old, new)


os.chdir(r'releases')
if os.path.exists('plugin.nro'):
    stream = os.popen('del plugin.nro')
    output = stream.read()
    output
stream = os.popen('rename libplugin.nro plugin.nro')
output = stream.read()
output
"""