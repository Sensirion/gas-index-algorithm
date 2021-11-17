from sensirion_gas_index_algorithm.voc_algorithm import VocAlgorithm


def test_voc_algorithm():
    algorithm = VocAlgorithm()
    algorithm_version = algorithm.get_version()
    assert isinstance(algorithm_version, str)
    out = algorithm.process(1)
    assert isinstance(out, int)


def test_voc_algorithm_get_set_states():
    algorithm = VocAlgorithm()
    algorithm.set_states(1, 2)
    state0, state1 = algorithm.get_states()
    assert state0 == 1
    assert state1 == 2


def test_voc_algorithm_get_set_tuning_parameters():
    algorithm = VocAlgorithm()
    algorithm.set_tuning_parameters(1, 2, 3, 4, 5, 6)
    assert [1, 2, 3, 4, 5, 6] == algorithm.get_tuning_parameters()


def test_voc_algorithm_process():
    algorithm = VocAlgorithm()
    # after a few 100 samples we should reach mean
    for n in range(200):
        algorithm.process(1337)
    index = algorithm.process(1337)
    assert index == 100
