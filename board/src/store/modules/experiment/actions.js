import types from './types';
import { query } from '@/services/Api';

export default {
  [types.FETCH_ALL] ({commit, state, rootState, dispatch, rootGetters}, id) {
    query("Experiment")
      .all()
      .then(res => {
        commit(types.FETCH_ALL, res.data)
      })
  },

  [types.DELETE] ({commit, state, rootState, dispatch, rootGetters}, id) {
    query("Experiment")
      .deleteCascade(id)
      .then(() => {
        dispatch(types.FETCH_ALL);
      })
  },
};
