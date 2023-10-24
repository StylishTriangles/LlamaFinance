import { reactive } from "vue";

class TestStore {
  counter = 0;

  init() {
    // Add anything that's needed at init
  }

  increment() {
    this.counter++;
  }
}

export const testStore = reactive(new TestStore()) as any as TestStore;
