/*
 * Copyright 2016-2018, 2020-2021 Uber Technologies, Inc.
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
/** @file coordijk.c
 * @brief   Hex IJK coordinate systems functions including conversions to/from
 * lat/lng.
 */

#include "coordijk.h"

/**
 * Rotates indexing digit 60 degrees counter-clockwise. Returns result.
 *
 * @param digit Indexing digit (between 1 and 6 inclusive)
 */
Direction _rotate60ccw(Direction digit) {
    switch (digit) {
        case K_AXES_DIGIT:
            return IK_AXES_DIGIT;
        case IK_AXES_DIGIT:
            return I_AXES_DIGIT;
        case I_AXES_DIGIT:
            return IJ_AXES_DIGIT;
        case IJ_AXES_DIGIT:
            return J_AXES_DIGIT;
        case J_AXES_DIGIT:
            return JK_AXES_DIGIT;
        case JK_AXES_DIGIT:
            return K_AXES_DIGIT;
        default:
            return digit;
    }
}

/**
 * Rotates indexing digit 60 degrees clockwise. Returns result.
 *
 * @param digit Indexing digit (between 1 and 6 inclusive)
 */
Direction _rotate60cw(Direction digit) {
    switch (digit) {
        case K_AXES_DIGIT:
            return JK_AXES_DIGIT;
        case JK_AXES_DIGIT:
            return J_AXES_DIGIT;
        case J_AXES_DIGIT:
            return IJ_AXES_DIGIT;
        case IJ_AXES_DIGIT:
            return I_AXES_DIGIT;
        case I_AXES_DIGIT:
            return IK_AXES_DIGIT;
        case IK_AXES_DIGIT:
            return K_AXES_DIGIT;
        default:
            return digit;
    }
}


