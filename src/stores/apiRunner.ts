import { defineStore } from "pinia";
import {
  readTextFile,
  BaseDirectory,
  createDir,
  writeFile,
} from "@tauri-apps/api/fs";
import { getClient } from "@tauri-apps/api/http";
import { invoke } from '@tauri-apps/api/tauri';

export const useApiRunnerStore = defineStore("apiRunner", {
  state: (): ApiRunnerState => ({
    currentRequestResult: null,
    currentRequestConfig: {
      key: null,
      name: "New Request",
      method: "get",
      url: "",
    },
    fullData: null,
  }),
  getters: {
    //computed
  },
  actions: {
    //methods
    async readFiles() {
      this.fullData = JSON.parse(
        await readTextFile("./Simple Postman/data.json", {
          dir: BaseDirectory.Document,
        })
      );
    },
    async createDataFolder() {
      try {
        await createDir("Simple Postman", {
          dir: BaseDirectory.Document,
          recursive: true,
        });
      } catch (error) {
        console.log(error);
      }
    },
    async storeDataFile() {
      try {
        if (this.fullData) {
          this.fullData?.map((item: any, index: any) => {
            if (item.key == this.currentRequestConfig.key) {
              this.fullData[index] = this.currentRequestConfig;
            }
          });
        }
        await this.createDataFolder();
        await this.saveJsonFile();
      } catch (error) {
        console.log(error);
      }
    },
    async saveJsonFile() {
      try {
        await writeFile(
          {
            contents: JSON.stringify(this.fullData),
            path: `./Simple Postman/data.json`,
          },
          {
            dir: BaseDirectory.Document,
          }
        );
      } catch (error) {
        console.log(error);
      }
    },
    async runApi() {
      let response = {} as Response;

      const client = await getClient();
      if (this.currentRequestConfig.method === "get") {
        response = await client.get(this.currentRequestConfig.url);
      } else if (this.currentRequestConfig.method === "post") {
        response = await client.get(this.currentRequestConfig.url);
      }

      this.currentRequestResult = response.data;
    },
    async addNewRequest() {
      this.fullData?.push({
        key: Date.now(),
        name: "New Request",
        method: "get",
        url: "",
      });
      await this.saveJsonFile();
    },
    async currentPageConfig(id: any) { 
      let data: RequestConfig = await invoke('get_request', {id: id})
      this.currentRequestConfig = { ...data }; 
    },
  },
});

interface ApiRunnerState {
  currentRequestResult: any;
  currentRequestConfig: RequestConfig | any;
  fullData: any;
}

interface RequestConfig {
  id: Number | null;
  folder_id: Number | null;
  name: String;
  method: String;
  url: String;
}

interface Response {
  data: Object;
}
