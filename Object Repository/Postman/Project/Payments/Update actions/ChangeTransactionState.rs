<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ChangeTransactionState</name>
   <tag></tag>
   <elementGuidId>59508284-2ea9-4a5e-9da9-cf5057a4189d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${payment-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;changeTransactionState\&quot;,\n            \&quot;transactionId\&quot; : \&quot;${transactionId}\&quot;,\n            \&quot;state\&quot; : \&quot;Failure\&quot;\n          }\n    ]\n}&quot;,
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
      <webElementGuid>d326b38d-997e-469f-88a0-6d521d197262</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c0c2826c-0c54-414b-9b70-fba588e3a61f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/payments/${payment-id}</restUrl>
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
      <id>f6300f30-3f5a-4f8b-b42f-0f7148a03c18</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>4d19bc72-5784-479e-912b-6022286685ce</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.payment-id</defaultValue>
      <description></description>
      <id>9a4ca03f-3222-40e4-af3d-7d55368f609b</id>
      <masked>false</masked>
      <name>payment-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.payment-version</defaultValue>
      <description></description>
      <id>07462fd0-453b-4dff-ab4b-5adba8ec8fb9</id>
      <masked>false</masked>
      <name>payment-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transactionId</defaultValue>
      <description></description>
      <id>00d8f980-05d5-47e7-8290-7167e08a41b8</id>
      <masked>false</masked>
      <name>transactionId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
