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
/** @file faceijk.h
 * @brief   FaceIJK functions including conversion to/from lat/lng.
 *
 *  References the Vec2d cartesian coordinate systems hex2d: local face-centered
 *     coordinate system scaled a specific H3 grid resolution unit length and
 *     with x-axes aligned with the local i-axes
 */

#ifndef FACEIJK_H
#define FACEIJK_H

#include "coordijk.h"

/** @struct FaceIJK
 * @brief Face number and ijk coordinates on that face-centered coordinate
 * system
 */
typedef struct {
    int face;        ///< face number
    CoordIJK coord;  ///< ijk coordinates on that face
} FaceIJK;

#endif
