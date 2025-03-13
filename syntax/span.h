#pragma once
#include "location.h"

class span
{
public:
    location* start;
    location* end;

    span(location* start, location* end);
};
