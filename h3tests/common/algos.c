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
/** @file algos.c
 * @brief   Hexagon grid algorithms
 */

#include "algos.h"
#include "baseCells.h"
#include "bbox.h"
#include "h3Assert.h"
#include "h3Index.h"
#include "h3api.h"

/**
 * New digit when traversing along class II grids.
 *
 * Current digit -> direction -> new digit.
 */
static const Direction NEW_DIGIT_II[7][7] = {
    {CENTER_DIGIT, K_AXES_DIGIT, J_AXES_DIGIT, JK_AXES_DIGIT, I_AXES_DIGIT,
     IK_AXES_DIGIT, IJ_AXES_DIGIT},
    {K_AXES_DIGIT, I_AXES_DIGIT, JK_AXES_DIGIT, IJ_AXES_DIGIT, IK_AXES_DIGIT,
     J_AXES_DIGIT, CENTER_DIGIT},
    {J_AXES_DIGIT, JK_AXES_DIGIT, K_AXES_DIGIT, I_AXES_DIGIT, IJ_AXES_DIGIT,
     CENTER_DIGIT, IK_AXES_DIGIT},
    {JK_AXES_DIGIT, IJ_AXES_DIGIT, I_AXES_DIGIT, IK_AXES_DIGIT, CENTER_DIGIT,
     K_AXES_DIGIT, J_AXES_DIGIT},
    {I_AXES_DIGIT, IK_AXES_DIGIT, IJ_AXES_DIGIT, CENTER_DIGIT, J_AXES_DIGIT,
     JK_AXES_DIGIT, K_AXES_DIGIT},
    {IK_AXES_DIGIT, J_AXES_DIGIT, CENTER_DIGIT, K_AXES_DIGIT, JK_AXES_DIGIT,
     IJ_AXES_DIGIT, I_AXES_DIGIT},
    {IJ_AXES_DIGIT, CENTER_DIGIT, IK_AXES_DIGIT, J_AXES_DIGIT, K_AXES_DIGIT,
     I_AXES_DIGIT, JK_AXES_DIGIT}};

/**
 * New traversal direction when traversing along class II grids.
 *
 * Current digit -> direction -> new ap7 move (at coarser level).
 */
static const Direction NEW_ADJUSTMENT_II[7][7] = {
    {CENTER_DIGIT, CENTER_DIGIT, CENTER_DIGIT, CENTER_DIGIT, CENTER_DIGIT,
     CENTER_DIGIT, CENTER_DIGIT},
    {CENTER_DIGIT, K_AXES_DIGIT, CENTER_DIGIT, K_AXES_DIGIT, CENTER_DIGIT,
     IK_AXES_DIGIT, CENTER_DIGIT},
    {CENTER_DIGIT, CENTER_DIGIT, J_AXES_DIGIT, JK_AXES_DIGIT, CENTER_DIGIT,
     CENTER_DIGIT, J_AXES_DIGIT},
    {CENTER_DIGIT, K_AXES_DIGIT, JK_AXES_DIGIT, JK_AXES_DIGIT, CENTER_DIGIT,
     CENTER_DIGIT, CENTER_DIGIT},
    {CENTER_DIGIT, CENTER_DIGIT, CENTER_DIGIT, CENTER_DIGIT, I_AXES_DIGIT,
     I_AXES_DIGIT, IJ_AXES_DIGIT},
    {CENTER_DIGIT, IK_AXES_DIGIT, CENTER_DIGIT, CENTER_DIGIT, I_AXES_DIGIT,
     IK_AXES_DIGIT, CENTER_DIGIT},
    {CENTER_DIGIT, CENTER_DIGIT, J_AXES_DIGIT, CENTER_DIGIT, IJ_AXES_DIGIT,
     CENTER_DIGIT, IJ_AXES_DIGIT}};

/**
 * New traversal direction when traversing along class III grids.
 *
 * Current digit -> direction -> new ap7 move (at coarser level).
 */
static const Direction NEW_DIGIT_III[7][7] = {
    {CENTER_DIGIT, K_AXES_DIGIT, J_AXES_DIGIT, JK_AXES_DIGIT, I_AXES_DIGIT,
     IK_AXES_DIGIT, IJ_AXES_DIGIT},
    {K_AXES_DIGIT, J_AXES_DIGIT, JK_AXES_DIGIT, I_AXES_DIGIT, IK_AXES_DIGIT,
     IJ_AXES_DIGIT, CENTER_DIGIT},
    {J_AXES_DIGIT, JK_AXES_DIGIT, I_AXES_DIGIT, IK_AXES_DIGIT, IJ_AXES_DIGIT,
     CENTER_DIGIT, K_AXES_DIGIT},
    {JK_AXES_DIGIT, I_AXES_DIGIT, IK_AXES_DIGIT, IJ_AXES_DIGIT, CENTER_DIGIT,
     K_AXES_DIGIT, J_AXES_DIGIT},
    {I_AXES_DIGIT, IK_AXES_DIGIT, IJ_AXES_DIGIT, CENTER_DIGIT, K_AXES_DIGIT,
     J_AXES_DIGIT, JK_AXES_DIGIT},
    {IK_AXES_DIGIT, IJ_AXES_DIGIT, CENTER_DIGIT, K_AXES_DIGIT, J_AXES_DIGIT,
     JK_AXES_DIGIT, I_AXES_DIGIT},
    {IJ_AXES_DIGIT, CENTER_DIGIT, K_AXES_DIGIT, J_AXES_DIGIT, JK_AXES_DIGIT,
     I_AXES_DIGIT, IK_AXES_DIGIT}};

/**
 * New traversal direction when traversing along class III grids.
 *
 * Current digit -> direction -> new ap7 move (at coarser level).
 */
static const Direction NEW_ADJUSTMENT_III[7][7] = {
    {CENTER_DIGIT, CENTER_DIGIT, CENTER_DIGIT, CENTER_DIGIT, CENTER_DIGIT,
     CENTER_DIGIT, CENTER_DIGIT},
    {CENTER_DIGIT, K_AXES_DIGIT, CENTER_DIGIT, JK_AXES_DIGIT, CENTER_DIGIT,
     K_AXES_DIGIT, CENTER_DIGIT},
    {CENTER_DIGIT, CENTER_DIGIT, J_AXES_DIGIT, J_AXES_DIGIT, CENTER_DIGIT,
     CENTER_DIGIT, IJ_AXES_DIGIT},
    {CENTER_DIGIT, JK_AXES_DIGIT, J_AXES_DIGIT, JK_AXES_DIGIT, CENTER_DIGIT,
     CENTER_DIGIT, CENTER_DIGIT},
    {CENTER_DIGIT, CENTER_DIGIT, CENTER_DIGIT, CENTER_DIGIT, I_AXES_DIGIT,
     IK_AXES_DIGIT, I_AXES_DIGIT},
    {CENTER_DIGIT, K_AXES_DIGIT, CENTER_DIGIT, CENTER_DIGIT, IK_AXES_DIGIT,
     IK_AXES_DIGIT, CENTER_DIGIT},
    {CENTER_DIGIT, CENTER_DIGIT, IJ_AXES_DIGIT, CENTER_DIGIT, I_AXES_DIGIT,
     CENTER_DIGIT, IJ_AXES_DIGIT}};

/**
 * Returns the hexagon index neighboring the origin, in the direction dir.
 *
 * Implementation note: The only reachable case where this returns 0 is if the
 * origin is a pentagon and the translation is in the k direction. Thus,
 * 0 can only be returned if origin is a pentagon.
 *
 * @param origin Origin index
 * @param dir Direction to move in
 * @param rotations Number of ccw rotations to perform to reorient the
 *                  translation vector. Will be modified to the new number of
 *                  rotations to perform (such as when crossing a face edge.)
 * @param out H3Index of the specified neighbor if succesful
 * @return E_SUCCESS on success
 */
H3Error h3NeighborRotations(H3Index origin, Direction dir, int *rotations,
                            H3Index *out) {
    H3Index current = origin;

    if (dir < CENTER_DIGIT || dir >= INVALID_DIGIT) {
        return E_FAILED;
    }
    // Ensure that rotations is modulo'd by 6 before any possible addition,
    // to protect against signed integer overflow.
    *rotations = *rotations % 6;
    for (int i = 0; i < *rotations; i++) {
        dir = _rotate60ccw(dir);
    }

    int newRotations = 0;
    int oldBaseCell = H3_GET_BASE_CELL(current);
    if (NEVER(oldBaseCell < 0) || oldBaseCell >= NUM_BASE_CELLS) {
        // Base cells less than zero can not be represented in an index
        return E_CELL_INVALID;
    }
    Direction oldLeadingDigit = _h3LeadingNonZeroDigit(current);

    // Adjust the indexing digits and, if needed, the base cell.
    int r = H3_GET_RESOLUTION(current) - 1;
    while (true) {
        if (r == -1) {
            H3_SET_BASE_CELL(current, baseCellNeighbors[oldBaseCell][dir]);
            newRotations = baseCellNeighbor60CCWRots[oldBaseCell][dir];

            if (H3_GET_BASE_CELL(current) == INVALID_BASE_CELL) {
                // Adjust for the deleted k vertex at the base cell level.
                // This edge actually borders a different neighbor.
                H3_SET_BASE_CELL(current,
                                 baseCellNeighbors[oldBaseCell][IK_AXES_DIGIT]);
                newRotations =
                    baseCellNeighbor60CCWRots[oldBaseCell][IK_AXES_DIGIT];

                // perform the adjustment for the k-subsequence we're skipping
                // over.
                current = _h3Rotate60ccw(current);
                *rotations = *rotations + 1;
            }

            break;
        } else {
            Direction oldDigit = H3_GET_INDEX_DIGIT(current, r + 1);
            Direction nextDir;
            if (oldDigit == INVALID_DIGIT) {
                // Only possible on invalid input
                return E_CELL_INVALID;
            } else if (isResolutionClassIII(r + 1)) {
                H3_SET_INDEX_DIGIT(current, r + 1, NEW_DIGIT_II[oldDigit][dir]);
                nextDir = NEW_ADJUSTMENT_II[oldDigit][dir];
            } else {
                H3_SET_INDEX_DIGIT(current, r + 1,
                                   NEW_DIGIT_III[oldDigit][dir]);
                nextDir = NEW_ADJUSTMENT_III[oldDigit][dir];
            }

            if (nextDir != CENTER_DIGIT) {
                dir = nextDir;
                r--;
            } else {
                // No more adjustment to perform
                break;
            }
        }
    }

    int newBaseCell = H3_GET_BASE_CELL(current);
    if (_isBaseCellPentagon(newBaseCell)) {
        int alreadyAdjustedKSubsequence = 0;

        // force rotation out of missing k-axes sub-sequence
        if (_h3LeadingNonZeroDigit(current) == K_AXES_DIGIT) {
            if (oldBaseCell != newBaseCell) {
                // in this case, we traversed into the deleted
                // k subsequence of a pentagon base cell.
                // We need to rotate out of that case depending
                // on how we got here.
                // check for a cw/ccw offset face; default is ccw

                if (ALWAYS(_baseCellIsCwOffset(
                        newBaseCell,
                        baseCellData[oldBaseCell].homeFijk.face))) {
                    current = _h3Rotate60cw(current);
                } else {
                    // See cwOffsetPent in testGridDisk.c for why this is
                    // unreachable.
                    current = _h3Rotate60ccw(current);
                }
                alreadyAdjustedKSubsequence = 1;
            } else {
                // In this case, we traversed into the deleted
                // k subsequence from within the same pentagon
                // base cell.
                if (oldLeadingDigit == CENTER_DIGIT) {
                    // Undefined: the k direction is deleted from here
                    return E_PENTAGON;
                } else if (oldLeadingDigit == JK_AXES_DIGIT) {
                    // Rotate out of the deleted k subsequence
                    // We also need an additional change to the direction we're
                    // moving in
                    current = _h3Rotate60ccw(current);
                    *rotations = *rotations + 1;
                } else if (oldLeadingDigit == IK_AXES_DIGIT) {
                    // Rotate out of the deleted k subsequence
                    // We also need an additional change to the direction we're
                    // moving in
                    current = _h3Rotate60cw(current);
                    *rotations = *rotations + 5;
                } else {
                    // TODO: Should never occur, but is reachable by fuzzer
                    return E_FAILED;
                }
            }
        }

        for (int i = 0; i < newRotations; i++)
            current = _h3RotatePent60ccw(current);

        // Account for differing orientation of the base cells (this edge
        // might not follow properties of some other edges.)
        if (oldBaseCell != newBaseCell) {
            if (_isBaseCellPolarPentagon(newBaseCell)) {
                // 'polar' base cells behave differently because they have all
                // i neighbors.
                if (oldBaseCell != 118 && oldBaseCell != 8 &&
                    _h3LeadingNonZeroDigit(current) != JK_AXES_DIGIT) {
                    *rotations = *rotations + 1;
                }
            } else if (_h3LeadingNonZeroDigit(current) == IK_AXES_DIGIT &&
                       !alreadyAdjustedKSubsequence) {
                // account for distortion introduced to the 5 neighbor by the
                // deleted k subsequence.
                *rotations = *rotations + 1;
            }
        }
    } else {
        for (int i = 0; i < newRotations; i++)
            current = _h3Rotate60ccw(current);
    }

    *rotations = (*rotations + newRotations) % 6;
    *out = current;

    return E_SUCCESS;
}

/**
 * _getEdgeHexagons takes a given geoloop ring (either the main geoloop or
 * one of the holes) and traces it with hexagons and updates the search and
 * found memory blocks. This is used for determining the initial hexagon set
 * for the polygonToCells algorithm to execute on.
 *
 * @param geoloop The geoloop (or hole) to be traced
 * @param numHexagons The maximum number of hexagons possible for the geoloop
 *                    (also the bounds of the search and found arrays)
 * @param res The hexagon resolution (0-15)
 * @param numSearchHexes The number of hexagons found so far to be searched
 * @param search The block of memory containing the hexagons to search from
 * @param found The block of memory containing the hexagons found from the
 * search
 *
 * @return An error code if the hash function cannot insert a found hexagon
 *         into the found array.
 */
H3Error _getEdgeHexagons(const GeoLoop *geoloop, int64_t numHexagons, int res,
                         int64_t *numSearchHexes, H3Index *search,
                         H3Index *found) {
    for (int i = 0; i < geoloop->numVerts; i++) {
        LatLng origin = geoloop->verts[i];
        LatLng destination = i == geoloop->numVerts - 1 ? geoloop->verts[0]
                                                        : geoloop->verts[i + 1];
        int64_t numHexesEstimate;
        H3Error estimateErr =
            lineHexEstimate(&origin, &destination, res, &numHexesEstimate);
        if (estimateErr) {
            return estimateErr;
        }
        for (int64_t j = 0; j < numHexesEstimate; j++) {
            LatLng interpolate;
            interpolate.lat =
                (origin.lat * (numHexesEstimate - j) / numHexesEstimate) +
                (destination.lat * j / numHexesEstimate);
            interpolate.lng =
                (origin.lng * (numHexesEstimate - j) / numHexesEstimate) +
                (destination.lng * j / numHexesEstimate);
            H3Index pointHex;
            H3Error e = latLngToCell(&interpolate, res, &pointHex);
            if (e) {
                return e;
            }
            // A simple hash to store the hexagon, or move to another place if
            // needed
            int64_t loc = (int64_t)(pointHex % numHexagons);
            int64_t loopCount = 0;
            while (found[loc] != 0) {
                // If this conditional is reached, the `found` memory block is
                // too small for the given polygon. This should not happen.
                // TODO: Reachable via fuzzer
                if (loopCount > numHexagons) return E_FAILED;
                if (found[loc] == pointHex)
                    break;  // At least two points of the geoloop index to the
                            // same cell
                loc = (loc + 1) % numHexagons;
                loopCount++;
            }
            if (found[loc] == pointHex)
                continue;  // Skip this hex, already exists in the found hash
            // Otherwise, set it in the found hash for now
            found[loc] = pointHex;

            search[*numSearchHexes] = pointHex;
            (*numSearchHexes)++;
        }
    }
    return E_SUCCESS;
}


