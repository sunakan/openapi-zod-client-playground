import { createApiClient, api } from "$lib/todo_api/todoApiClient";
import { isErrorFromAlias} from "@zodios/core";

export async function load() {
  const client = createApiClient('http://127.0.0.1:3000');
  try {
    //return await client.getTodo({params: {id: 405}});
    return await client.getTodo({id: 405});
    //return await client.updateTodo({params: {id: 404}, });
  } catch (error) {
    console.log(error);
    if (isErrorFromAlias(api.api, "getTodo", error)) {
      console.log('エラーハンドリング');
      console.log(error.response.data);
    }
  }
}
