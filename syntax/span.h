#pragma once
#include "location.h"

class span
{
public:
    location start;
    location end;

    span(const location& start, const location& end);
};
