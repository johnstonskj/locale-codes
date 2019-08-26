import math
import pandas as pd
import re
import sys

def read_data():
    frame = pd.read_csv('script-names.csv', header=0)
    scripts = []
    for row in frame.itertuples():
        scripts.append({
            'alphabetic_code': row.Code,
            'numeric_code': row.No,
            'name': row.English_Name,
            'alias': row.Alias
        })
    return scripts

def write_data_out(scripts, out_path):
    rows = map(
        lambda cinfo:
        '"%s":{%s}' % (
            cinfo['alphabetic_code'],
            ','.join([
                '"alphabetic_code":"%s"' % cinfo['alphabetic_code'],
                '"numeric_code":%s' % cinfo['numeric_code'],
                '"name":"%s"' % cinfo['name'],
                '"alias":%s' % optional_string(cinfo['alias'])
            ])),
        scripts)
    print('writing %s/scripts.json' % out_path)
    with open('%s/scripts.json' % out_path, 'w') as text_file:
        print('{%s}' % ','.join(rows), file=text_file)

def optional_string(s):
    return ('"%s"' % s) if isinstance(s, str) else 'null'

if len(sys.argv) < 2:
    print('Error: need a path argument')
else:
    write_data_out(read_data(), sys.argv[1])
