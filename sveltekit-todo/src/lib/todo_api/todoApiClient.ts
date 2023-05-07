import { makeApi, Zodios, type ZodiosOptions } from "@zodios/core";
import { z } from "zod";

const TodoRequest = z.object({ title: z.string(), is_completed: z.boolean() });
const TodoResponse = TodoRequest.and(
  z.object({
    id: z.number().int(),
    created_at: z.string(),
    updated_at: z.string(),
  })
);
const NotFoundError = z.object({ code: z.string(), message: z.string() });
const UnknownError = z.object({ code: z.string(), message: z.string() });

export const schemas = {
  TodoRequest,
  TodoResponse,
  NotFoundError,
  UnknownError,
};

export const endpoints = makeApi([
  {
    method: "get",
    path: "/todos/:id",
    alias: "getTodo",
    requestFormat: "json",
    parameters: [
      {
        name: "id",
        type: "Path",
        schema: z.number().int(),
      },
    ],
    response: TodoResponse,
    errors: [
      {
        status: 404,
        description: `Not Found`,
        schema: NotFoundError,
      },
      {
        status: 500,
        description: `Unknown error`,
        schema: UnknownError,
      },
    ],
  },
  {
    method: "put",
    path: "/todos/:id",
    alias: "updateTodo",
    requestFormat: "json",
    parameters: [
      {
        name: "body",
        description: `Todo note`,
        type: "Body",
        schema: TodoRequest,
      },
      {
        name: "id",
        type: "Path",
        schema: z.number().int(),
      },
    ],
    response: TodoResponse,
    errors: [
      {
        status: 404,
        description: `Not Found`,
        schema: NotFoundError,
      },
      {
        status: 500,
        description: `Unknown error`,
        schema: UnknownError,
      },
    ],
  },
]);

export const api = new Zodios(endpoints);

export function createApiClient(baseUrl: string, options?: ZodiosOptions) {
  return new Zodios(baseUrl, endpoints, options);
}
