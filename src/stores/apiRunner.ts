import { defineStore } from 'pinia'

export const useApiRunnerStore = defineStore('apiRunner', {
    state: () => ({
        result: '',
        json: {},
    }),
    getters: {
        //computed
    },
    actions: {
        //methods
    }
})