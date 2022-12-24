/*
 * Copyright 2019-2021 Uber Technologies, Inc.
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

#include <inttypes.h>
#include <stdlib.h>

#include "h3Index.h"
#include "test.h"

SUITE(cellToCenterChild) {
    H3Index baseHex;
    LatLng baseCentroid;
    setH3Index(&baseHex, 8, 4, 2);
    cellToLatLng(baseHex, &baseCentroid);

    TEST(propertyTests) {
        for (int res = 0; res <= MAX_H3_RES - 1; res++) {
            for (int childRes = res + 1; childRes <= MAX_H3_RES; childRes++) {
                LatLng centroid;
                H3Index h3Index;
                t_assertSuccess(latLngToCell(&baseCentroid, res, &h3Index));
                cellToLatLng(h3Index, &centroid);

                H3Index geoChild;
                t_assertSuccess(latLngToCell(&centroid, childRes, &geoChild));
                H3Index centerChild;
                t_assertSuccess(cellToCenterChild(h3Index, childRes, &centerChild));

                t_assert(
                    centerChild == geoChild,
                    "center child should be same as indexed centroid at child "
                    "resolution");
                t_assert(getResolution(centerChild) == childRes,
                         "center child should have correct resolution");
                H3Index parent;
                t_assertSuccess(cellToParent(centerChild, res, &parent));
                t_assert(
                    parent == h3Index,
                    "parent at original resolution should be initial index");
            }
        }
    }

    TEST(sameRes) {
        int res = getResolution(baseHex);
        H3Index child;
        t_assertSuccess(cellToCenterChild(baseHex, res, &child));
        t_assert(child == baseHex,
                 "center child at same resolution should return self");
    }

    TEST(invalidInputs) {
        int res = getResolution(baseHex);
        H3Index child;
        t_assert(cellToCenterChild(baseHex, res - 1, &child) ==
                     E_RES_DOMAIN,
                 "should fail at coarser resolution");
        t_assert(
            cellToCenterChild(baseHex, -1, &child) == E_RES_DOMAIN,
            "should fail for negative resolution");
        t_assert(cellToCenterChild(baseHex, MAX_H3_RES + 1,
                                              &child) == E_RES_DOMAIN,
                 "should fail beyond finest resolution");
    }
}
