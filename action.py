import argparse
import json
import os
import subprocess
import urllib
import urllib.request


def main(arch, dep_name):

    with urllib.request.urlopen(urllib.request.Request(
        url=f'https://crates.io/api/v1/crates/{dep_name}',
        headers={'User-Agent': '@Windows81 on GitHub'},
    )) as response:
        data = json.loads(response.read())

    versions = [
        v['num']
        for v in data['versions']
    ]

    subprocess.call(['rustup', 'target', 'add', arch])

    for version in versions:
        path = f'results/{arch}-{dep_name}-{version}.md'
        if os.path.exists(path):
            continue

        add_ret = subprocess.call(
            ['cargo', 'add', f'{dep_name}@={version}'],
        )

        if (add_ret > 0):
            continue

        compile_ret = subprocess.call([
            'cargo', 'rustc', '--release', '--target', arch, '--features', f'feat-{dep_name}', '--no-default-features',
            '--', '-C', 'opt-level=3', '-C', 'link-dead-code=y', '-C', 'panic=abort',
        ])

        if compile_ret > 0:
            continue

        subprocess.call(['cargo', 'run', '--release'], stdout=open(path, 'w'))

        if os.stat(path).st_size == 0:
            os.remove(path)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('arch')
    parser.add_argument('dep_name')
    main(**parser.parse_args().__dict__)
