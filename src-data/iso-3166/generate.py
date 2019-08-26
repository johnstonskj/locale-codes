import math
import pandas as pd
import sys

def read_data():
    data_frame = pd.read_csv('all.csv', header=0)

    regions = {}

    countries = []

    for row in data_frame.itertuples():
        regions[int(row.country_code)] = row.name
        if not math.isnan(row.region_code):
            regions[int(row.region_code)] = row.region
        if not math.isnan(row.sub_region_code):
            regions[int(row.sub_region_code)] = row.sub_region
        if not math.isnan(row.intermediate_region_code):
            regions[int(row.intermediate_region_code)] = row.intermediate_region

        countries.append({
            'code': row.alpha_3,
            'short': row.alpha_2,
            'country': int(row.country_code),
            'region': None if math.isnan(row.region_code) else int(row.region_code),
            'sub_region': None if math.isnan(row.sub_region_code) else int(row.sub_region_code),
            'intermediate': None if math.isnan(row.intermediate_region_code) else int(row.intermediate_region_code)
        })
    return (regions, countries)

def write_data(regions, countries, out_path):
    r_rows = map(
        lambda rinfo: '"%s":"%s"' % (rinfo[0], rinfo[1]),
        regions.items())
    print('writing %s/regions.json' % out_path)
    with open('%s/regions.json' % out_path, 'w') as text_file:
        print('{%s}' % ','.join(r_rows), file=text_file)

    c_rows = map(
        lambda cinfo:
           '"%s":{%s}' % (
                cinfo['code'],
                ','.join([
                    '"code":"%s"' % cinfo['code'],
                    '"short_code":"%s"' % cinfo['short'],
                    '"country_code":%s' % cinfo['country'],
                    '"region_code":%s' % ('null' if cinfo['region'] is None else '%s' % cinfo['region']),
                    '"sub_region_code":%s' % ('null' if cinfo['sub_region'] is None else '%s' % cinfo['sub_region']),
                    '"intermediate_region_code":%s' % ('null' if cinfo['intermediate'] is None else '%s' % cinfo['intermediate'])
                ])),
        countries)
    print('writing %s/countries.json' % out_path)
    with open('%s/countries.json' % out_path, 'w') as text_file:
        print('{%s}' % ','.join(c_rows), file=text_file)

if len(sys.argv) < 2:
    print('Error: need a path argument')
else:
    write_data(*read_data(), sys.argv[1])