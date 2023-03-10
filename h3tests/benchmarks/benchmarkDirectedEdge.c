/*
 * Copyright 2020-2021 Uber Technologies, Inc.
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
#include "benchmark.h"
#include "h3api.h"
#include "latLng.h"

// Fixtures (arbitrary res 9 hexagon)
H3Index edges[6] = {0};
H3Index hex = 0x89283080ddbffff;

BEGIN_BENCHMARKS();

CellBoundary outBoundary;
originToDirectedEdges(hex, edges);

BENCHMARK(directedEdgeToBoundary, 10000, {
    for (int i = 0; i < 6; i++) {
        directedEdgeToBoundary(edges[i], &outBoundary);
    }
});

END_BENCHMARKS();
