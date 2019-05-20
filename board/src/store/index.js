import Vuex from 'vuex';
import Vue from 'vue';
import createPersistedState from 'vuex-persistedstate';

import * as experimentStore from './experimentStore';
import * as appStore from './appStore';
import * as loadingStore from "./loadingStore";
import * as metricStore from "./metricStore";
import * as traceStore from "./traceStore";
import * as tracePointStore from "./tracePointStore";
import * as traceGroupStore from "./traceGroupStore";

Vue.use(Vuex);

export default new Vuex.Store({
  plugins: [
    createPersistedState()
  ],
  modules: {
    [experimentStore.namespace]: experimentStore.store,
    [appStore.namespace]: appStore.store,
    [loadingStore.namespace]: loadingStore.store,
    [metricStore.namespace]: metricStore.store,
    [traceStore.namespace]: traceStore.store,
    [tracePointStore.namespace]: tracePointStore.store,
    [traceGroupStore.namespace]: traceGroupStore.store,
  },
});
