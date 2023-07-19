#!/usr/bin/env python
from lorenz_equations_plotter import solve_multiple

from time import time

import plotly.express as px
import plotly.graph_objects as go
import plotly.io as pio

pio.templates.default = "plotly_dark"

INIT_POINT1 = (1,2,3)
INIT_POINT2 = (1,2,3)

STEP = 0.0001
STOP = 100.

σ = 10
b = 8/3
r = 28

start = time()

trajectories = solve_multiple(
        (INIT_POINT1, INIT_POINT2),
        STOP, 
        STEP,
        (σ, b, r)
        )

end = time()
print(f"Time taken back in python: {end-start}")

trace1 = go.Scatter3d(x=trajectories[0][0],
                      y=trajectories[0][1],
                      z=trajectories[0][2],
                      mode="lines",
                      name=f"{INIT_POINT1}",
                      showlegend=True,
                      line = dict(color="green"),
                      )

trace2 = go.Scatter3d(x=trajectories[1][0],
                      y=trajectories[1][1],
                      z=trajectories[1][2],
                      mode="lines",
                      name=f"{INIT_POINT1}",
                      showlegend=True,
                      line = dict(color="cyan"),
                      )
