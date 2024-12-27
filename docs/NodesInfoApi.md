# \NodesInfoApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_nodes_info**](NodesInfoApi.md#get_nodes_info) | **GET** /1.0/kb/nodesInfo | Retrieve all the nodes infos
[**trigger_node_command**](NodesInfoApi.md#trigger_node_command) | **POST** /1.0/kb/nodesInfo | Trigger a node command



## get_nodes_info

> Vec<models::NodeInfo> get_nodes_info()
Retrieve all the nodes infos

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NodeInfo>**](NodeInfo.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_node_command

> trigger_node_command(x_killbill_created_by, body, local_node_only, x_killbill_reason, x_killbill_comment)
Trigger a node command

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_killbill_created_by** | **String** |  | [required] |
**body** | [**NodeCommand**](NodeCommand.md) |  | [required] |
**local_node_only** | Option<**bool**> |  |  |[default to false]
**x_killbill_reason** | Option<**String**> |  |  |
**x_killbill_comment** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

