#include "dog.hpp"
#include <cstdio>

Dog::Dog(const char *name) :
    _name (name), _status (STATUS_STOP)
{
    std::printf("%s: おはよう、わん！\n", _name);
}

Dog::~Dog()
{
    std::printf("%s: おやすみ、ゎん．\n", _name);
}

void Dog::walk()
{
    std::printf("%s: わんわん！\n", _name);
    if (_status != STATUS_WALKING) _status = STATUS_WALKING;
}

void Dog::stop()
{
    std::printf("%s: わん．\n", _name);
    if (_status != STATUS_STOP) _status = STATUS_STOP;
}

