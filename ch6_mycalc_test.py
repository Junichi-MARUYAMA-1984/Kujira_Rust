# Pythonで動的ライブラリを利用する
import platform, os
from ctypes import cdll

# PythonでOSを判定
pf = platform.system()
print(pf)

# Windowsの場合
if pf == 'Windows': 
    libfile = 'ch6_mycalc.dll'
# macOSの場合
elif pf == 'Darwin':
    libfile = 'libch6_mycalc.dylib'
# Linuxの場合
else:
    libfile = 'libch6_mycalc.so'

# 動的ライブラリのパスを指定
libpath = os.path.join(os.path.dirname(__file__), libfile)
print("lib=", libpath)

# ライブラリをロード
mycalc = cdll.LoadLibrary(libpath)

# Rustのライブラリを実行
print(mycalc.rust_mul(100, 8))
print(mycalc.rust_mul(8, 9))

