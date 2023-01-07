#include <iostream>
#include <vector>
#include <cstdio>

using namespace std;

using ll = int64_t;

struct result {ll ans; ll worst_path;};

result dsf(ll x, const vector<vector<ll>>& g, const vector<ll>& F) {
    if (g[x].size() == 0) {
        return result{F[x],F[x]};
    } else {
        ll ans = 0;
        ll worst_path = 1e9;

        for (ll c : g[x]) {
            auto [total, path] = dsf(c, g, F);
            ans += total;
            worst_path = min(worst_path, path);
        }
        if (worst_path < F[x]) {
            ans += F[x] - worst_path;
            worst_path = F[x];
        }
        return result{ans, worst_path};
    }
}

int main() {
    ll T;
    scanf("%lld", &T);
    for (ll tcase=1; tcase<= T; tcase++) {
        ll n;
        scanf("%lld", &n);
        
        vector<ll> F(n+1,0);
        for (ll i=0; i<n; i++) {
            // cin >> F[i+1];
            scanf("%lld", &F[i+1]);
        }

        vector<ll> P(n+1,-1);
        for (ll i=0; i<n; i++) {
            scanf("%lld", &P[i+1]);
        }

        vector<vector<ll>> graph(n+1,vector<ll>{});
        for (ll i=0; i<=n; i++) {
            if (P[i] >= 0) {
                graph[P[i]].push_back(i);
            }
        }
 
        auto [ans,root_path] = dsf(0,graph,F);
        printf("Case #%lld: %lld \n",tcase,ans);
    }

    return 0;
}