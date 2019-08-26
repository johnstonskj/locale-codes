import math
import pandas as pd
import re
import sys

def read_data():
    data_frame = pd.read_csv('people.tsv', sep='\t', header=0)
    people = []
    for row in data_frame.itertuples():
        people.append(row._1)

    data_frame = pd.read_csv('character-sets-1.csv', header=0)
    character_sets = []
    for row in data_frame.itertuples():
        character_sets.append({
            'name': row.Name,
            'aliases': re.split("\n+", str(row.Aliases)),
            'mib_code': row.MIBenum,
            'source': squish(un_newline(row.Source)),
            'reference': squish(un_newline(row.Reference))
        })

    return (character_sets, people)

def write_data(character_sets, people, out_path):
    rows = map(
        lambda cinfo:
           '"%s":{%s}' % (
                cinfo['name'],
                ','.join([
                    '"name":"%s"' % cinfo['name'],
                    '"also_known_as":%s' % vector(cinfo['aliases']),
                    '"mib_code":%s' % cinfo['mib_code'],
                    '"source":%s' % optional_str(unquote(cinfo['source'])),
                    '"reference":%s' %  optional_str(unquote(cinfo['reference']))
                ])),
        character_sets)
    # TODO: filter people out of source/reference fields
    print('writing %s/codesets.json' % out_path)
    with open('%s/codesets.json' % out_path, 'w') as text_file:
        print('{%s}' % ','.join(rows), file=text_file)

def un_newline(s):
    return s.replace('\n', ' ') if isinstance(s, str) else s

def squish(s):
    return re.sub(' +', ' ', s) if isinstance(s, str) else s

def optional_str(s):
    return '"%s"' % s if isinstance(s, str) else 'null'

def unquote(s):
    return s.replace('"', '\'') if isinstance(s, str) else s

def vector(ss):
    return '[%s]' % ','.join(
        map(
            lambda s: '"%s"' % s,
            ss))

if len(sys.argv) < 2:
    print('Error: need a path argument')
else:
    write_data(*read_data(), sys.argv[1])
