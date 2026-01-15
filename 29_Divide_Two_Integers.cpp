// 29 divide-two-integers
#include <bits/stdc++.h>
using namespace std;

template<typename T>
ostream& operator<<(ostream& os, const vector<T>& v) {
    os << "[";
    for(int i=0; i<v.size(); ++i) {
        os << v[i];
        if(i < v.size()-1) os << ",";
    }
    os << "]";
    return os;
}

int divide(int dividend, int divisor) {
        return dividend/divisor;
    }

int main() {
    vector<tuple<int, int>> testcases = {
        { 10, 3 },
        { 7, -3 },
    };

    vector<int> answers = {
        3,
        -2,
    };

    int t = testcases.size();
    for(int i = 0; i < t; i++) {
        int ans = std::apply(divide, testcases[i]);
        if(ans == answers[i]) {
            cout << "Testcase " << i+1 << " passed!\n";
        } else {
            cout << "Testcase " << i+1 << " failed!\n";
            cout << "Expected: " << answers[i] << "\n";
            cout << "Got: " << ans << "\n";
            break;
        }
    }
    return 0;
}
