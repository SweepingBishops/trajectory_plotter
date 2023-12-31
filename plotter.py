#!/usr/bin/env python
from lorenz_equations_plotter import solve_multiple, mult_parameter

from time import time

import plotly.express as px
import plotly.graph_objects as go
import plotly.io as pio

pio.templates.default = "plotly_dark"


INIT_POINTS = (
    #(5,50,50),
    (0,1,0),
    #(-5,50,50),
    #(-2.5,50,50),
    (1,1,1),
    (1,1,-1),
    (1,-1,1),
    (-1,1,1),
    (1,0,0),
    (-1,0,0),
    (0,1,0),
    (0,-1,0),
    (0,0,1),
    (0,0,-1),
)


STEP = 0.001
STOP = 100.

σ = 10
b = 8/3
r = 24.5

params = (
        (10,8/3,14.05),
        (10,8/3,14.0501),
        (10,8/3,14.0502),
        (10,8/3,14.0503),
        (10,8/3,14.0504),
        (10,8/3,14.0505),
        (10,8/3,14.0506),
        (10,8/3,14.0507),
        (10,8/3,14.0508),
        )

init = (0,1,0)

init_iter = iter(INIT_POINTS)
params_iter = iter(params)

start = time()

trajectories = solve_multiple(
        INIT_POINTS,
        STOP, 
        STEP,
        (σ, b, r),
        )

# trajectories = mult_parameter(
#         init,
#         STOP, 
#         STEP,
#         params,
#         )


end = time()
print(f"Time taken back in python: {end-start}")

traces = list()

for trajectory in trajectories:
    traces.append(
            go.Scatter3d(x=trajectory[0],
                         y=trajectory[1],
                         z=trajectory[2],
                         mode="lines",
                         name=f"{next(init_iter)}",
                         showlegend=True,
                         )
            )

# traces.append(go.Scatter3d(x=[0,],y=[0,],z=[0,],
#               mode="markers",
#               name=f"Origin",
#               showlegend=True,
#               marker=dict(color="Green"),
#               ))

fig = go.Figure(data=traces)
#fig.show()
fig.write_html("plot.html")
