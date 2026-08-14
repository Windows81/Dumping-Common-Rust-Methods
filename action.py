import argparse
import json
import os
import subprocess
import urllib
import urllib.request


def main(arch, dep_name):

    with urllib.request.urlopen(urllib.request.Request(
        url='https://crates.io/api/v1/crates/reqwest',
        headers={'User-Agent': '@Windows81 on GitHub'},
    )) as response:
        data = json.loads(response.read())

    versions = [
        v['num']
        for v in data['versions']
    ]

    with open('Cargo.toml', 'a') as o:
        o.write("[features]\n")
        o.write(f"default = [\"dep_{dep_name}\"]\n")
        o.write(f"dep_{dep_name} = []\n")

    for version in versions:
        path = f'results/{arch}-{dep_name}-{version}-$line.txt'
        if os.path.exists(path):
            continue

        subprocess.call(
            ['cargo', 'add', f'{dep_name}@{version}'],
        )
        subprocess.call([
            'cargo', 'rustc', '--release', '--target', arch,
            '--', '-C', 'opt-level=3', '-C', 'link-dead-code=y', '-C', 'panic=abort',
        ])
        subprocess.call(['cargo', 'run', '--release'], stdout=open(path, 'w'))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('arch')
    parser.add_argument('dep_name')
    main(**parser.parse_args().__dict__)
