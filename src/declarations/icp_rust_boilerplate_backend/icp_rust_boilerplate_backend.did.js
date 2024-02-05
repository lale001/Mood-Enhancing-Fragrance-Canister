export const idlFactory = ({ IDL }) => {
  const FragrancePayload = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'mood_enhancing_properties' : IDL.Vec(IDL.Text),
  });
  const Fragrance = IDL.Record({
    'id' : IDL.Nat64,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'name' : IDL.Text,
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
    'mood_enhancing_properties' : IDL.Vec(IDL.Text),
  });
  const Error = IDL.Variant({ 'NotFound' : IDL.Record({ 'msg' : IDL.Text }) });
  const Result = IDL.Variant({ 'Ok' : Fragrance, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Vec(Fragrance), 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Text), 'Err' : Error });
  return IDL.Service({
    'add_fragrance' : IDL.Func([FragrancePayload], [IDL.Opt(Fragrance)], []),
    'delete_fragrance' : IDL.Func([IDL.Nat64], [Result], []),
    'filter_fragrances_by_mood' : IDL.Func([IDL.Text], [Result_1], ['query']),
    'get_fragrance' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'get_recommendations' : IDL.Func([IDL.Text], [Result_1], ['query']),
    'list_fragrances' : IDL.Func([], [IDL.Vec(Fragrance)], ['query']),
    'search_fragrance_names' : IDL.Func([IDL.Text], [Result_2], ['query']),
    'sort_fragrances_by_creation_date' : IDL.Func(
        [],
        [IDL.Vec(Fragrance)],
        ['query'],
      ),
    'update_fragrance' : IDL.Func([IDL.Nat64, FragrancePayload], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
