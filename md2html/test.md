
### A


#### 题解

题目即让我们求满足$sumA[i]+sumB[j]\leq K$的$max(i+j)$，$sumA[i]$表示从$A$顶部拿$i$本书的代价，同理$B$；那么只需要枚举$i$，二分找到满足$sumA[i]+sumB[j]\leq K(sumB[j] \leq K - sumA[i])$的最大的$j$；然后更新$max(i+j)$即可；

#### code
```cpp
#include <iostream>
#include <algorithm>
#include <cmath>
#include  <vector>
#define clr(a, b) memset(a, b, sizeof(a))
#define inf 0x3f3f3f3f

using namespace std;

typedef long long ll;
const int maxn = 200005;

int n, m;
ll k;

ll s1[maxn], s2[maxn];

int find(ll x) {
    int l = -1, r = m + 1;
    while (l < r - 1) {
        int mid = (l + r) >>1 ;
        if (x + s2[mid] <= k)
            l = mid;
        else
            r = mid;
    }
    return l;
}

int main() {
    // freopen("test.in", "r", stdin);
    // freopen("test.out", "w", stdout);
    cin >> n >> m >> k;
    int t;
    for (int i = 1; i <= n; i++)
        cin >> t, s1[i] = s1[i - 1] + t;
    for (int j = 1; j <= m; j++)
        cin >> t, s2[j] = s2[j - 1] + t;
    int res = 0, maxm = 0;
    for (int i = 0; i <= n; i++) {
        if (s1[i] > k) break;
        maxm = i;
        int pos = find(s1[i]);
        maxm += pos;
        res = max(maxm, res);
    }
    cout << res;
}
```

### B

可以暴力求出每个数的因子数直接算；也可以分别算出每个数的贡献；比如对于$1$，那么它在$f(1)\dots f(n)$，出现n次，贡献为$N(N+1)/2$，对于$i$，$N$个数里面有$t=\left \lfloor \frac{N}{i}\right \rfloor$个数将$i$作为因子；贡献为$\frac{t\*(t+1)\*i}{2}$；

```cpp
#include <iostream>
#define clr(a, b) memset(a, b, sizeof(a))
#define inf 0x3f3f3f3f

using namespace std;

typedef long long ll;
const int maxn = 200005;

ll res;
int n;
int main() {
    // freopen("test.in", "r", stdin);
    // freopen("test.out", "w", stdout);
    cin >> n;
    
    for (int i = 1; i <= n; i++) {
        int t = n / i;
        res += 1ll * t * (t + 1) / 2 * i;
    }
    cout << res << endl;
}
```

```cpp
#include <bits/stdc++.h>
using namespace std;
 
long long divi[10000002] = {0};
 
int main(){
    int N;
    cin >> N;
    for(int i=1;i<=N;i++){
        for(int j=i;j<=N;j+=i){
            divi[j]++;
        }
    }
 
    long long ans = 0;
    for(int i=1;i<=N;i++){
        ans += i * divi[i];
    }
    cout << ans << endl;
}
```

### C

#### 题解

用最少的时间段覆盖整个大区间；这是一个贪心问题；对于一个已经选择的时间段$T$，那么我们应该尽可能希望下一个选择的时间段的右边界尽量的大，当然前提是该时间段的左边界在上一个时间段右边界的左边；

#### code

```cpp
#include <stdio.h>
#include <string.h>
#include <algorithm>
 
#define maxn 25005
 
struct Node {
	int u, v;
} E[maxn];
 
bool cmp(Node a, Node b) {
	return a.u < b.u;
}
 
int main() {
	int N, T, flag, ans, right, i;
	while(scanf("%d%d", &N, &T) == 2) {
		for(i = 0; i < N; ++i)
			scanf("%d%d", &E[i].u, &E[i].v);
		std::sort(E, E + N, cmp);
		ans = right = 0;
		flag = 1;
		i = 0;
		while(flag <= T) {
			for( ; i < N && E[i].u <= flag; ++i)
				if(E[i].u <= flag && E[i].v > right)
					right = E[i].v;
			if(right >= flag) flag = right + 1, ++ans;
			else break;
		}
		if(flag <= T) ans = -1;
		printf("%d\n", ans);
	}
	return 0;
}
```

### D

#### 题解

每个人都被分到一个笔和笔记本输出$Yes$，即$n \leq m, n\leq k$；

#### code
```cpp
#include <iostream>
using namespace std;

int n, m ,k;
int main() {
    cin >> n >> m >> k;
    if (n <= m and n <= k)
        cout << "Yes" << endl;
    else
        cout << "No" << endl;
}
```

$\begin{aligned} G(x) &= (1 + x)(1 + x^2)(1 + x^4)(1 + x^8)(1 + x^{16})(1 +x^{32})\\
                      &=(1 + x + x^2 + \dots + x^{63})\\
                      &=\sum_{k=0}^{63}x^k
\end{aligned}$

$\begin{aligned} G_1(x) & = (1 + x + x^2 + \dots)(1 + x^2 + x^4 + \dots)\dots (1 +x^m + x^{2m} + \dots)\\
                        & = \frac{1}{1-x} \frac{1}{1-x^2} \dots \frac{1}{1-x^m}
\end{aligned}$