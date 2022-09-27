import { createStore } from "vuex";
import { ProjectInfo } from "../lib/common";
import { AppConfig } from "../lib/setting";

interface StoreData {
    appConfig: AppConfig | null,
    isFirstRunning: boolean | null,
    histories: History[],
    openedProject: ProjectInfo,
}

const store = createStore({
    state() {
        return <StoreData>{
            appConfig: null,
            isFirstRunning: null,
            histories: [],
            openedProject: { name: "", path: "" }
        };
    },
    mutations: {
        is_first_running(state, is: boolean) {
            state.isFirstRunning = is;
        },
        set_config(state, config: AppConfig) {
            state.appConfig = config;
        },
        set_projects_home(state, home: string) {
            if (state.appConfig === null) {
            } else {
                state.appConfig.projects_home = home;
            }
        },
        set_histories(state, list: History[]) {
            state.histories = list;
        },
        set_opened(state, rojectInfo: ProjectInfo) {
            state.openedProject = rojectInfo;
        }
    }

});

export default store;

