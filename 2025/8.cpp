#include <bits/stdc++.h>
using namespace std;

const int N = 1000;

vector<int> parent, size;

int find_parent(int a) {
    if (parent[a] == a) {
        return a;
    }

    parent[a] = find_parent(parent[a]);
    return parent[a];
}

void merge(int a, int b) {
    int aa = find_parent(a);
    int bb = find_parent(b);

    if (aa != bb) {
        parent[bb] = aa;
        size[aa] += size[bb];
    }
}

int main() {
    parent.resize(N);
    size.resize(N);
    for (int i = 0; i < N; i++) {
        parent[i] = i;
        size[i] = 1;
    }

    char comma;
    vector<int> x(N), y(N), z(N);
    for (int i = 0; i < N; i++) {
        cin >> x[i] >> comma >> y[i] >> comma >> z[i];
    }

    vector<pair<long long,pair<int,int>>> dist;
    for (int i = 0; i < N; i++) {
        for (int j = i + 1; j < N; j++) {
            long long xx = (x[i] - x[j]) * 1LL * (x[i] - x[j]);
            long long yy = (y[i] - y[j]) * 1LL * (y[i] - y[j]);
            long long zz = (z[i] - z[j]) * 1LL * (z[i] - z[j]);
            dist.push_back(make_pair(xx + yy + zz, make_pair(i, j)));
        }
    }

    sort(dist.begin(), dist.end());

    long long last_x = 0;
    int times = 1000;
    for (int i = 0; i < times; i++) {
        int a = dist[i].second.first;
        int b = dist[i].second.second;
        if (find_parent(a) == find_parent(b)) {
            continue;
        }

        merge(a, b);
        last_x = x[a] * x[b];
    }

    // part 1
    vector<int> top(3, 0);
    for (int i = 0; i < N; i++) {
        int a = find_parent(i);
        if (a == i && size[a] > top[0]) {
            top[0] = size[a];
            sort(top.begin(), top.end());
        }
    }

    // part 2
    for (int i = times; i < dist.size(); i++) {
        int a = dist[i].second.first;
        int b = dist[i].second.second;
        if (find_parent(a) == find_parent(b)) {
            continue;
        }

        merge(a, b);
        last_x = x[a] * x[b];
    }

    cout << top[0] << ' ' << top[1] << ' ' << top[2] << '\n';
    cout << top[0] * top[1] * top[2] << '\n';
    cout << last_x << '\n';

    return 0;
}