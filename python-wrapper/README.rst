Sensirion Gas Index Algorithm
=============================

SGP4x VOC/NOx Engine Algorithm applies a gain-offset normalization algorithm to a SGP tick signal. The algorithm 
assumes a humidity compensated raw tick signal, applies state estimation, tick-to-GasIndex conversion and an 
adaptive lowpass filter.

The main goal of the VOC/NOX Engine algorithm is to calculate a VOC/NOx Index signal that enables robust detection of ambient
VOC/NOx changes, with minimal sensor-to-sensor variations. The Algorithm Engine calculates the VOC/NOx Index signal 
recursively from a single raw tick value Sout that is measured by the SGP sensor at each time step, as well as internal 
states that are updated at each time step. These internal states are most importantly the recursively estimated mean and 
variance of the Sout signal, as well as some additional internal states such as uptime and other counters.


Install
-------
.. sourcecode:: bash

    pip install sensirion-gas-index-algorithm

Recommended usage is within a virtualenv.

Usage VOC Algorithm
-------------------
.. sourcecode:: python

    from sensirion_gas_index_algorithm.voc_algorithm import VocAlgorithm

    voc_algorithm = VocAlgorithm()
    s_voc_raw = 27000  # read out from SGP41
    for _ in range(100):
        voc_index = voc_algorithm.process(s_voc_raw)
        print(voc_index)


Usage NOx Algorithm
-------------------
.. sourcecode:: python

    from sensirion_gas_index_algorithm.nox_algorithm import NoxAlgorithm

    nox_algorithm = NoxAlgorithm()
    s_nox_raw = 18000  # read out from SGP41
    for _ in range(100):
        nox_index = nox_algorithm.process(s_voc_raw)
        print(nox_index)
