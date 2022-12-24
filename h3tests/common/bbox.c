/*
 * Copyright 2016-2021 Uber Technologies, Inc.
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
/** @file bbox.c
 * @brief   Geographic bounding box functions
 */

#include "bbox.h"

#include <math.h>

/**
 * _hexRadiusKm returns the radius of a given hexagon in Km
 *
 * @param h3Index the index of the hexagon
 * @return the radius of the hexagon in Km
 */
double _hexRadiusKm(H3Index h3Index) {
    // There is probably a cheaper way to determine the radius of a
    // hexagon, but this way is conceptually simple
    LatLng h3Center;
    CellBoundary h3Boundary;
    cellToLatLng(h3Index, &h3Center);
    cellToBoundary(h3Index, &h3Boundary);
    return greatCircleDistanceKm(&h3Center, h3Boundary.verts);
}

/**
 * lineHexEstimate returns an estimated number of hexagons that trace
 *                 the cartesian-projected line
 *
 * @param origin the origin coordinates
 * @param destination the destination coordinates
 * @param res the resolution of the H3 hexagons to trace the line
 * @param out Out parameter for the estimated number of hexagons required to
 * trace the line
 * @return E_SUCCESS (0) on success or another value otherwise.
 */
H3Error lineHexEstimate(const LatLng *origin, const LatLng *destination,
                        int res, int64_t *out) {
    // Get the area of the pentagon as the maximally-distorted area possible
    H3Index pentagons[12] = {0};
    H3Error pentagonsErr = getPentagons(res, pentagons);
    if (pentagonsErr) {
        return pentagonsErr;
    }
    double pentagonRadiusKm = _hexRadiusKm(pentagons[0]);

    double dist = greatCircleDistanceKm(origin, destination);
    double distCeil = ceil(dist / (2 * pentagonRadiusKm));
    if (!isfinite(distCeil)) {
        return E_FAILED;
    }
    int64_t estimate = (int64_t)distCeil;
    if (estimate == 0) estimate = 1;
    *out = estimate;
    return E_SUCCESS;
}
