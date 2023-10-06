# Utility Scripts

Scripts made to autofill the README.md files by fetching the information directly from the website via API.

For the scripts to work, you must follow the [Guidelines](../GUIDELINE.md) on creating new solution for problems.

## How to Setup

Requirements:
* Python3
* requests
* markdownify

To install requests, simply type:
```python
pip install requests
# if you have multiple python versions
pip3 install requests

# or if pip isn't in your path
python -m pip install requests
# if you have multiple python versions
python3 -m pip install requests
```

Following the same as before, to install markdownify:
```python
pip install markdownify
# if you have multiple python versions
pip3 install markdownify

# or if pip isn't in your path
python -m pip install markdownify
# if you have multiple python versions
python3 -m pip install markdownify
```

Installing the dependencies in a virtual environment is encouraged.


## Running the Script

the main script that you should use in order to fully update the contents of the all the README.md files is:

* `insert_problems_descriptions.py`
    * Responsible for creating the readme for each problem that currently don't have a readme. Aswell as updating the website folder with all the problems in a list below is how to use it:
        * `python ./scripts/insert_problems_descriptions.py`
