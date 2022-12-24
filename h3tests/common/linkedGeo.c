/*
 * Copyright 2017-2018, 2020-2021 Uber Technologies, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *         http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/** @file linkedGeo.c
 * @brief   Linked data structure for geo data
 */

#include "linkedGeo.h"
#include "h3api.h"

/**
 * Count the number of polygons in a linked list
 * @param  polygon Starting polygon
 * @return         Count
 */
int countLinkedPolygons(LinkedGeoPolygon *polygon) {
    int count = 0;
    while (polygon != NULL) {
        count++;
        polygon = polygon->next;
    }
    return count;
}

/**
 * Count the number of linked loops in a polygon
 * @param  polygon Polygon to count loops for
 * @return         Count
 */
int countLinkedLoops(LinkedGeoPolygon *polygon) {
    LinkedGeoLoop *loop = polygon->first;
    int count = 0;
    while (loop != NULL) {
        count++;
        loop = loop->next;
    }
    return count;
}

/**
 * Count the number of coordinates in a loop
 * @param  loop Loop to count coordinates for
 * @return      Count
 */
int countLinkedCoords(LinkedGeoLoop *loop) {
    LinkedLatLng *coord = loop->first;
    int count = 0;
    while (coord != NULL) {
        count++;
        coord = coord->next;
    }
    return count;
}
