import ruamel.yaml
import argparse

parser = argparse.ArgumentParser(description='conda_environment.yml -> pip_requirements.txt')
parser.add_argument('environment_yml_file', type=str, metavar='environment.yml', default='environment.yml', help='the input file')
parser.add_argument('-o', type=str, metavar='pip_requirements.txt', default=None, help='the output file')
args = parser.parse_args()
basepath = args.environment_yml_file.replace("\\",'/').rsplit('/', maxsplit=1)[0]
out_file = args.o
if out_file == None:
    out_file = basepath + '/' + 'pip_requirements.txt'
print(args.environment_yml_file, '->', out_file)

yaml = ruamel.yaml.YAML()
data = yaml.load(open(args.environment_yml_file))

requirements = []
for dep in data['dependencies']:
    if isinstance(dep, str):
        arr = dep.split('=')
        package = arr[0]
        package_version = arr[1]
        if len(arr) == 3:
            python_version = arr[2]
            if python_version == '0':
                continue
        if package != 'python':
            requirements.append(package + '==' + package_version)
    elif isinstance(dep, dict):
        for preq in dep.get('pip', []):
            requirements.append(preq)

with open(out_file, 'w') as fp:
    for requirement in requirements:
       print(requirement, file=fp)

print('Intall dependencies within the right python environment using:')
print('pip install -r '+out_file)