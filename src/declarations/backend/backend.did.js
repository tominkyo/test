export const idlFactory = ({ IDL }) => {
  const Status = IDL.Variant({
    'InvalidInput' : IDL.Null,
    'Success' : IDL.Null,
    'UnAuthorized' : IDL.Null,
  });
  const UpdateRespone = IDL.Record({ 'status' : Status, 'message' : IDL.Text });
  const CreateFileData = IDL.Record({
    'id' : IDL.Vec(IDL.Nat8),
    'name' : IDL.Text,
    'children' : IDL.Opt(IDL.Vec(IDL.Text)),
    'parent_id' : IDL.Vec(IDL.Nat8),
    'directory_id' : IDL.Vec(IDL.Nat8),
  });
  const ElementId = IDL.Variant({
    'None' : IDL.Null,
    'Some' : IDL.Vec(IDL.Nat8),
  });
  const FileNode = IDL.Record({
    'name' : IDL.Text,
    'element_tree' : ElementId,
  });
  const Tree = IDL.Record({
    'vertices' : IDL.Vec(IDL.Tuple(IDL.Vec(IDL.Nat8), FileNode)),
    'adjacency' : IDL.Vec(
      IDL.Tuple(IDL.Vec(IDL.Nat8), IDL.Vec(IDL.Vec(IDL.Nat8)))
    ),
  });
  const FileDirectory = IDL.Opt(
    IDL.Variant({ 'id' : IDL.Vec(IDL.Nat8), 'files' : Tree, 'name' : IDL.Text })
  );
  const QueryUser = IDL.Opt(
    IDL.Record({
      'username' : IDL.Opt(IDL.Text),
      'image' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    })
  );
  const User = IDL.Record({ 'image' : IDL.Vec(IDL.Nat8) });
  return IDL.Service({
    'create_directory' : IDL.Func([], [UpdateRespone], []),
    'create_file' : IDL.Func([CreateFileData], [], []),
    'get_directories' : IDL.Func([], [FileDirectory], ['query']),
    'get_profile' : IDL.Func([], [QueryUser], ['query']),
    'register' : IDL.Func([IDL.Text], [UpdateRespone], []),
    'test_ic' : IDL.Func([], [IDL.Text], ['query']),
    'update_profile' : IDL.Func([User], [UpdateRespone], []),
  });
};
export const init = ({ IDL }) => { return []; };
