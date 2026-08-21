import functools
import urllib.request
import subprocess
import argparse
import urllib
import json
import os


@functools.cache
def versiontuple(v):
    t = v.split('-', 1)[0].split('.')
    return tuple(map(int, t))


def main(arch, dep_name, min_ver, max_ver):

    with urllib.request.urlopen(urllib.request.Request(
        url=f'https://crates.io/api/v1/crates/{dep_name}',
        headers={'User-Agent': '@Windows81 on GitHub'},
    )) as response:
        data = json.loads(response.read())

    versions = [
        v['num']
        for v in reversed(data['versions'])
    ]

    subprocess.call(['rustup', 'target', 'add', arch])

    for version in versions:
        if versiontuple(version) < versiontuple(min_ver):
            continue

        path = f'results/{arch}-{dep_name}-{version}.md'
        if os.path.exists(path):
            continue

        add_ret = subprocess.call(
            ['cargo', 'add', f'{dep_name}@={version}'],
        )

        if add_ret > 0:
            continue

        compile_ret = subprocess.call([
            'cargo', 'run', '--release', '--target', arch,
            '--features', f'feat-{dep_name}', '--no-default-features',
            '--', path,
        ])

        if compile_ret > 0:
            continue

        if os.stat(path).st_size == 0:
            os.remove(path)

        print(f'--- Program output dumped to {path} ---')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('arch')
    parser.add_argument('dep_name')
    parser.add_argument('min_ver')
    parser.add_argument('max_ver')
    main(**parser.parse_args().__dict__)
