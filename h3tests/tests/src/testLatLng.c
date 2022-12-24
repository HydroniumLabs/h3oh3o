/*
 * Copyright 2017-2021 Uber Technologies, Inc.
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
/** @file testLatLng.c
 * @brief Tests geographic coordinate functions
 *
 * usage: `testLatLng`
 */

#include <float.h>
#include <math.h>

#include "constants.h"
#include "h3api.h"
#include "latLng.h"
#include "test.h"
#include "utility.h"

/**
 * Test a function for all resolutions, where the value should be decreasing as
 * resolution increases.
 *
 * @param function
 * @param message
 */
static void testDecreasingFunction(H3Error (*function)(int, double *),
                                   const char *message) {
    double last = 0;
    double next;
    for (int i = MAX_H3_RES; i >= 0; i--) {
        t_assertSuccess(function(i, &next));
        t_assert(next > last, message);
        last = next;
    }
}

SUITE(latLng) {
    TEST(radsToDegs) {
        double originalRads = 1;
        double degs = radsToDegs(originalRads);
        double rads = degsToRads(degs);
        t_assert(fabs(rads - originalRads) < EPSILON_RAD,
                 "radsToDegs/degsToRads invertible");
    }

    TEST(distanceRads) {
        LatLng p1;
        setGeoDegs(&p1, 10, 10);
        LatLng p2;
        setGeoDegs(&p2, 0, 10);

        // TODO: Epsilon is relatively large
        t_assert(
            greatCircleDistanceRads(&p1, &p1) < EPSILON_RAD * 1000,
            "0 distance as expected");
        t_assert(fabs(greatCircleDistanceRads(&p1, &p2) -
                      degsToRads(10)) < EPSILON_RAD * 1000,
                 "distance along longitude as expected");
    }

    TEST(geoAlmostEqualThreshold) {
        LatLng a = {15, 10};
        LatLng b = {15, 10};
        t_assert(geoAlmostEqualThreshold(&a, &b, DBL_EPSILON), "same point");

        b.lat = 15.00001;
        b.lng = 10.00002;
        t_assert(geoAlmostEqualThreshold(&a, &b, 0.0001),
                 "differences under threshold");

        b.lat = 15.00001;
        b.lng = 10;
        t_assert(!geoAlmostEqualThreshold(&a, &b, 0.000001),
                 "lat over threshold");

        b.lat = 15;
        b.lng = 10.00001;
        t_assert(!geoAlmostEqualThreshold(&a, &b, 0.000001),
                 "lng over threshold");
    }

    TEST(distanceRads_wrappedLongitude) {
        const LatLng negativeLongitude = {.lat = 0, .lng = -(M_PI + M_PI_2)};
        const LatLng zero = {.lat = 0, .lng = 0};

        t_assert(fabs(M_PI_2 - greatCircleDistanceRads(
                                   &negativeLongitude, &zero)) < EPSILON_RAD,
                 "Distance with wrapped longitude");
        t_assert(fabs(M_PI_2 - greatCircleDistanceRads(
                                   &zero, &negativeLongitude)) < EPSILON_RAD,
                 "Distance with wrapped longitude and swapped arguments");
    }

    TEST(doubleConstants) {
        // Simple checks for ordering of values
        testDecreasingFunction(getHexagonAreaAvgKm2,
                               "getHexagonAreaAvgKm2 ordering");
        testDecreasingFunction(getHexagonAreaAvgM2,
                               "getHexagonAreaAvgM2 ordering");
        testDecreasingFunction(getHexagonEdgeLengthAvgKm,
                               "getHexagonEdgeLengthAvgKm ordering");
        testDecreasingFunction(getHexagonEdgeLengthAvgM,
                               "getHexagonEdgeLengthAvgM ordering");
    }

    TEST(doubleConstantsErrors) {
        double out;
        t_assert(getHexagonAreaAvgKm2(-1, &out) == E_RES_DOMAIN,
                 "getHexagonAreaAvgKm2 resolution negative");
        t_assert(getHexagonAreaAvgKm2(16, &out) == E_RES_DOMAIN,
                 "getHexagonAreaAvgKm2 resolution too high");
        t_assert(getHexagonAreaAvgM2(-1, &out) == E_RES_DOMAIN,
                 "getHexagonAreaAvgM2 resolution negative");
        t_assert(getHexagonAreaAvgM2(16, &out) == E_RES_DOMAIN,
                 "getHexagonAreaAvgM2 resolution too high");
        t_assert(getHexagonEdgeLengthAvgKm(-1, &out) == E_RES_DOMAIN,
                 "getHexagonEdgeLengthAvgKm resolution negative");
        t_assert(getHexagonEdgeLengthAvgKm(16, &out) == E_RES_DOMAIN,
                 "getHexagonEdgeLengthAvgKm resolution too high");
        t_assert(getHexagonEdgeLengthAvgM(-1, &out) == E_RES_DOMAIN,
                 "getHexagonEdgeLengthAvgM resolution negative");
        t_assert(getHexagonEdgeLengthAvgM(16, &out) == E_RES_DOMAIN,
                 "getHexagonEdgeLengthAvgM resolution too high");
    }

    TEST(intConstants) {
        // Simple checks for ordering of values
        int64_t last = 0;
        int64_t next;
        for (int i = 0; i <= MAX_H3_RES; i++) {
            t_assertSuccess(getNumCells(i, &next));
            t_assert(next > last, "getNumCells ordering");
            last = next;
        }
    }

    TEST(intConstantsErrors) {
        int64_t out;
        t_assert(getNumCells(-1, &out) == E_RES_DOMAIN,
                 "getNumCells resolution negative");
        t_assert(getNumCells(16, &out) == E_RES_DOMAIN,
                 "getNumCells resolution too high");
    }

    TEST(numHexagons) {
        // Test numHexagon counts of the number of *cells* at each resolution
        static const int64_t expected[] = {122L,
                                           842L,
                                           5882L,
                                           41162L,
                                           288122L,
                                           2016842L,
                                           14117882L,
                                           98825162L,
                                           691776122L,
                                           4842432842L,
                                           33897029882L,
                                           237279209162L,
                                           1660954464122L,
                                           11626681248842L,
                                           81386768741882L,
                                           569707381193162L};

        for (int r = 0; r <= MAX_H3_RES; r++) {
            int64_t num;
            t_assertSuccess(getNumCells(r, &num));
            t_assert(num == expected[r], "incorrect numHexagons count");
        }
    }
}
