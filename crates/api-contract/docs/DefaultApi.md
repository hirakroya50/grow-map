# \DefaultApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**execute_graph_ql**](DefaultApi.md#execute_graph_ql) | **POST** /graphql | Execute one GraphQL query or mutation; see schema.graphql
[**get_open_api**](DefaultApi.md#get_open_api) | **GET** /openapi.json | Return committed HTTP contract
[**get_swagger_ui**](DefaultApi.md#get_swagger_ui) | **GET** /docs | Swagger UI, enabled only in local/staging
[**liveness**](DefaultApi.md#liveness) | **GET** /health/live | Process liveness; independent of database health
[**readiness**](DefaultApi.md#readiness) | **GET** /health/ready | Required dependencies available within deadlines
[**refresh_session**](DefaultApi.md#refresh_session) | **POST** /v1/auth/sessions/refresh | Rotate refresh token; reuse revokes session family
[**request_wallet_challenge**](DefaultApi.md#request_wallet_challenge) | **POST** /v1/auth/challenges | Create single-use five-minute signing challenge
[**revoke_session**](DefaultApi.md#revoke_session) | **DELETE** /v1/auth/session | Revoke current session family
[**upgrade_graph_ql_web_socket**](DefaultApi.md#upgrade_graph_ql_web_socket) | **GET** /graphql/ws | Upgrade to graphql-transport-ws
[**verify_wallet_signature**](DefaultApi.md#verify_wallet_signature) | **POST** /v1/auth/sessions | Verify exact signature and atomically consume challenge



## execute_graph_ql

> models::GraphQlResponse execute_graph_ql(graph_ql_request)
Execute one GraphQL query or mutation; see schema.graphql

Canonical typed operations are in contracts/schema.graphql. Optional HTTP auth does not grant protected field access. Valid domain execution errors return HTTP 200 with errors. Invalid GraphQL document/validation returns 400 with GraphQL envelope. One mutation root field/request; no batching. Limit body 256 KiB.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**graph_ql_request** | [**GraphQlRequest**](GraphQlRequest.md) |  | [required] |

### Return type

[**models::GraphQlResponse**](GraphQLResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/graphql-response+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_open_api

> std::collections::HashMap<String, serde_json::Value> get_open_api()
Return committed HTTP contract

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_swagger_ui

> String get_swagger_ui()
Swagger UI, enabled only in local/staging

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## liveness

> models::Health liveness()
Process liveness; independent of database health

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Health**](Health.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## readiness

> models::Health readiness()
Required dependencies available within deadlines

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Health**](Health.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_session

> models::SessionResponse refresh_session(refresh_request)
Rotate refresh token; reuse revokes session family

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_request** | [**RefreshRequest**](RefreshRequest.md) |  | [required] |

### Return type

[**models::SessionResponse**](SessionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_wallet_challenge

> models::ChallengeResponse request_wallet_challenge(challenge_request)
Create single-use five-minute signing challenge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**challenge_request** | [**ChallengeRequest**](ChallengeRequest.md) |  | [required] |

### Return type

[**models::ChallengeResponse**](ChallengeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_session

> revoke_session()
Revoke current session family

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_graph_ql_web_socket

> upgrade_graph_ql_web_socket(upgrade, sec_web_socket_protocol)
Upgrade to graphql-transport-ws

Authentication is in connection_init payload authorization, within five seconds. Do not use token URL parameters. OpenAPI describes only the HTTP upgrade. Subscription fields and payloads are in schema.graphql; protocol semantics in docs/architecture.md. Close 4401/4403 for auth failure, 4408 for initialization timeout.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upgrade** | **String** |  | [required] |
**sec_web_socket_protocol** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_wallet_signature

> models::SessionResponse verify_wallet_signature(session_request)
Verify exact signature and atomically consume challenge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_request** | [**SessionRequest**](SessionRequest.md) |  | [required] |

### Return type

[**models::SessionResponse**](SessionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

