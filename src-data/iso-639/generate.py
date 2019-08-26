import pandas as pd
import sys

def read_data_from(created_date):
    tl_frame = pd.read_csv('iso-639-1-names.csv', sep=',', header=0)
    tl_names = {}
    for row in tl_frame.itertuples():
        tl_names[row.ID] = {
            'english': row.English_Name,
            'indigenous': row.Indigenous_Name
        }

    macros = pd.read_csv('iso-639-3-macrolanguages_%s.tab' % created_date, sep='\t', header=0)
    macro_languages = {}
    for row in macros.itertuples():
        if row.I_Status == 'A':
            parent = row.M_Id
            if parent not in macro_languages:
                macro_languages[parent] = []
            child = row.I_Id
            if isinstance(child, str):
                macro_languages[parent].append(child)

    languages = []
    main = pd.read_csv('iso-639-3_%s.tab' % created_date, sep='\t', header=0)
    for row in main.itertuples():
        children = []
        if row.Scope == 'M':
            children = macro_languages[row.Id]
        languages.append({
            'id': row.Id,
            'name': row.Ref_Name,
            'b_id': row.Part2B,
            't_id': row.Part2T,
            'sid': row.Part1,
            'scope': row.Scope,
            'l_type': row.Language_Type,
            'children': children,
        })
    return (languages, tl_names)

scope_values = {
    'I': 'Individual',
    'M': 'MacroLanguage',
    'S': 'Special'
}

type_values = {
    'A': 'Ancient',
    'C': 'Constructed',
    'E': 'Extinct',
    'H': 'Historical',
    'L': 'Living',
    'S': 'Special'
}

def write_data_out(languages, tl_names, out_path):
    rows = map(
        lambda linfo:
        '"%s":{%s}' % (
            linfo['id'],
            ','.join([
                '"code":"%s"' % linfo['id'],
                '"reference_name":"%s"' % clean(linfo['name']),
                '"indigenous_name":%s' % indigenous_name(linfo['sid'], tl_names),
                '"other_names":%s' % other_names(linfo['sid'], tl_names),
                '"bibliographic_code":%s' % optional_string(linfo['b_id']),
                '"terminology_code":%s' % optional_string(linfo['t_id']),
                '"short_code":%s' % optional_string(linfo['sid']),
                '"scope":"%s"' % scope_values[linfo['scope']],
                '"l_type":"%s"' % type_values[linfo['l_type']],
                '"family_members":%s' % optional_vector(linfo['children']),
            ])),
        languages)
    print('writing %s/languages.json' % out_path)
    with open('%s/languages.json' % out_path, 'w') as text_file:
        print('{%s}' % ','.join(rows), file=text_file)

def clean(s):
    return s.strip().replace('"', r'\"')

def other_names(key, map):
    if isinstance(key, str) and key in map:
        names =  map[key]['english'].split(';')
        if len(names) > 1:
            return optional_vector(names[1:])
    return 'null'

def indigenous_name(key, map):
    if isinstance(key, str) and key in map:
        return '"%s"' % clean(map[key]['indigenous'])
    else:
        return 'null'

def optional_string(s):
    return ('"%s"' % s) if isinstance(s, str) else 'null'

def optional_vector(v):
    if len(v) == 0:
        return 'null'
    else:
        return '[%s]' % ','.join(list(map(lambda x: '"%s"' % clean(x), v)))

if len(sys.argv) < 2:
    print('Error: need a path argument')
else:
    write_data_out(*read_data_from('20190408'), sys.argv[1])