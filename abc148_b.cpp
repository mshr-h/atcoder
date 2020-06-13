#include <iostream>

int main()
{
    int n;
    std::cin >> n;

    std::string s, t;
    std::cin >> s >> t;

    for (auto i = 0; i < n; i++)
    {
        std::cout << s[i] << t[i];
    }
    std::cout << std::endl;
}
