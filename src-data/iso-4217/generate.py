import math
import pandas as pd
import re
import sys

def read_data():
    frame = pd.read_excel('list_one.xls', skiprows=3, header=0)
    currencies = {}
    for row in frame.itertuples():
        if row._3 in currencies:
            currencies[row._3]['standards_entities'].append(row.ENTITY)
        else:
            currencies[row._3] = {
                'alphabetic_code': row._3,
                'name': row.Currency,
                'numeric_code': row._4,
                'symbol': None,
                'standards_entities': [row.ENTITY],
                'minor_units': row._5
            }

    sub_divisions = {}
    data_set = pd.read_html('forex-currency-codes.html', header=0)
    for frame in data_set:
        for row in frame.itertuples():
            sub_divisions[row.Alphabetic_code] = row.Subdivision

    symbols = {}
    frame = pd.read_csv('currency-symbols-ex.csv', header=0)
    for row in frame.itertuples():
        if row._3 in currencies:
            currencies[row._3]['symbol'] = row._6
        symbols[row._3] = row._6

    return (currencies.values(), sub_divisions, symbols)

def write_data_out(currencies, sub_divisions, symbols, out_path):
    rows = map(
        lambda cinfo:
        '"%s":{%s}' % (
            cinfo['alphabetic_code'],
            ','.join([
                '"alphabetic_code":"%s"' % cinfo['alphabetic_code'],
                '"name":"%s"' % cinfo['name'],
                '"numeric_code":%s' % optional_number(cinfo['numeric_code']),
                '"symbol":%s' % optional_string(cinfo['symbol']),
                '"standards_entities":[%s]' % standards_entities_list(cinfo['standards_entities']),
                '"subdivisions":[%s]' % sub_division_list(sub_divisions.get(cinfo['alphabetic_code'], math.nan), cinfo['minor_units'])
            ])),
        currencies)
    print('writing %s/currencies.json' % out_path)
    with open('%s/currencies.json' % out_path, 'w') as text_file:
        print('{%s}' % ','.join(rows), file=text_file)

def clean(s):
    return s.strip().replace('"', r'\"')

def optional_number(n):
    return 'null' if math.isnan(n) else int(n)

def optional_string(s):
    return ('"%s"' % s) if isinstance(s, str) else 'null'

def standards_entities_list(entities):
    new_list = []
    for entity in entities:
        if '_' in entity:
            new_list.append(entity)
        else:
            words = clean(entity).split(' ')
            new_list.append(
                ' '.join(list(map(lambda x: x.capitalize(), words)))
            )
    return ','.join(list(map(lambda x: '"%s"' % x, new_list)))

def sub_division_list(subs, minor):
    sub_divs = []
    if not isinstance(subs, str) and isinstance(minor, str) and minor.isdigit():
        sub_divs.append('{"exponent":%s,"name":null}' % minor)
    elif isinstance(subs, str):
        if not subs  == '-none-' and not subs.endswith('used)'):
            parts = subs.split('(')
            parts_2 = parts[0].strip().split(' ')
            exponent = parts_2[0].count('0')
            if parts_2[1] == 'new':
                sub_divs.append('{"exponent":%s,"name":"%s"}' % (exponent, 'new %s' % parts_2[2]))
            else:
                # rappen/centimes
                names = parts_2[1].split('/')
                for name in names:
                    sub_divs.append('{"exponent":%s,"name":"%s"}' % (exponent, name))
            if len(parts_2) == 5 and parts_2[2] in ['=', 'or']:
                sub_divs.append('{"exponent":%s,"name":"%s"}' % (parts_2[3].count('0'), parts_2[4]))
            another = '' if len(parts) == 1 else parts[1]
            if another.startswith('a.k.a'):
                another = another[1][7:-1]
            elif len(another) > 2:
                another = another[:-1]
            if another:
                sub_divs.append('{"exponent":%s,"name":"%s"}' % (exponent, another))
    return ','.join(sub_divs)

if len(sys.argv) < 2:
    print('Error: need a path argument')
else:
    write_data_out(*read_data(), sys.argv[1])
