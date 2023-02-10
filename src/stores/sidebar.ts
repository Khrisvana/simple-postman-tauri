import { defineStore } from "pinia";
import { invoke } from '@tauri-apps/api/tauri';
const invoke = window.__TAURI__.invoke;

export const useSidebarStore = defineStore("sidebar", {
  state: (): SidebarState => ({
    records: [],
  }),
  getters: {},
  actions: {
    async getRecords() {
      this.records = await invoke('map_requests');
    },
    updateElements(payload: Array<Record>) {
      this.records = payload;
    },
  },
});

interface SidebarState {
  records: Array<Record>;
}

interface Record {
  id: Number;
  parent_id: Number | null;
  name: String;
  requests: Array<Request>;
  items: Array<Record>;
}

interface Request {
  id: Number;
  method: String;
  String: String;
  url: String;
}
