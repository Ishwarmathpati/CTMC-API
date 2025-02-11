<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetStagedActions</name>
   <tag></tag>
   <elementGuidId>2054ac7f-2b52-4b13-b951-4a90e777a9d8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${edit-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setStagedActions\&quot;,\n            \&quot;stagedActions\&quot; : [ {\n              \&quot;action\&quot; : \&quot;setCountry\&quot;,\n              \&quot;country\&quot; : \&quot;DE\&quot;\n            } ]\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>497c661b-8d0c-43a8-8224-be205e651f52</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>618efb6d-8c31-4525-a955-92085e5215fd</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/orders/edits/${edit-id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>7ed2253b-2efd-4087-81bc-e007b79eaad4</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>eba6503a-2fab-4a23-80ec-aae29969d88b</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.edit-id</defaultValue>
      <description></description>
      <id>970ce17f-488d-43d3-9d8b-2cdca48fb756</id>
      <masked>false</masked>
      <name>edit-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.edit-version</defaultValue>
      <description></description>
      <id>a34f4851-a881-4f2b-a4b9-5661fcc41d72</id>
      <masked>false</masked>
      <name>edit-version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
