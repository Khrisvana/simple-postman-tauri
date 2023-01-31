import { defineStore } from "pinia";

export const useSidebarStore = defineStore("sidebar", {
  state: (): SidebarState => ({
    elements: [
      {
        id: 1,
        name: "Shrek",
        elements: [],
      },
      {
        id: 2,
        name: "Fiona",
        elements: [
          {
            id: 4,
            name: "Lord Farquad",
            elements: [],
          },
          {
            id: 5,
            name: "Prince Charming",
            elements: [],
          },
        ],
      },
      {
        id: 3,
        name: "Donkey",
        elements: [],
      },
    ],
  }),
  getters: {},
  actions: {
    updateElements(payload: Array<Elements>) {
      this.elements = payload;
    },
  },
});

interface SidebarState {
  elements: Array<Elements>;
}

interface Elements {
  id: Number;
  name: String;
  elements: Array<Elements>;
}
