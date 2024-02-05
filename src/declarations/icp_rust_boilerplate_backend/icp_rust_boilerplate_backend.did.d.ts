import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type Error = { 'NotFound' : { 'msg' : string } };
export interface Fragrance {
  'id' : bigint,
  'updated_at' : [] | [bigint],
  'name' : string,
  'description' : string,
  'created_at' : bigint,
  'mood_enhancing_properties' : Array<string>,
}
export interface FragrancePayload {
  'name' : string,
  'description' : string,
  'mood_enhancing_properties' : Array<string>,
}
export type Result = { 'Ok' : Fragrance } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Array<Fragrance> } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : Array<string> } |
  { 'Err' : Error };
export interface _SERVICE {
  'add_fragrance' : ActorMethod<[FragrancePayload], [] | [Fragrance]>,
  'delete_fragrance' : ActorMethod<[bigint], Result>,
  'filter_fragrances_by_mood' : ActorMethod<[string], Result_1>,
  'get_fragrance' : ActorMethod<[bigint], Result>,
  'get_recommendations' : ActorMethod<[string], Result_1>,
  'list_fragrances' : ActorMethod<[], Array<Fragrance>>,
  'search_fragrance_names' : ActorMethod<[string], Result_2>,
  'sort_fragrances_by_creation_date' : ActorMethod<[], Array<Fragrance>>,
  'update_fragrance' : ActorMethod<[bigint, FragrancePayload], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
