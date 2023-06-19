run:
  ./venv/bin/python ./testpy/test.py
venv:
  [ -d venv ] || python -m venv venv
activate: venv
  source ./venv/bin/activate

  
  
  
