set shell := ["powershell.exe", "-c"]
run: 
  @echo "run py"
  . .venv/Scripts/python ./testpy/test.py
  
