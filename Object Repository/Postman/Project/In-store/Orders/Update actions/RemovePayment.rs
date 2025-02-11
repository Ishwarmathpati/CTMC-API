<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>RemovePayment</name>
   <tag></tag>
   <elementGuidId>6812092e-77ff-495c-baa7-8800967d0c11</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${order-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;removePayment\&quot;,\n            \&quot;payment\&quot; : {\n              \&quot;typeId\&quot; : \&quot;payment\&quot;,\n              \&quot;id\&quot; : \&quot;${payment-id}\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>19237634-86e1-4d0b-980c-59e62872b312</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1f2b302d-7db8-4e90-9e99-21128fcdf112</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/in-store/key=${store-key}/orders/${order-id}</restUrl>
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
      <id>f05e5fe5-f3ce-4dff-9861-a727505dfa27</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>726e2e9d-a998-4b3c-9d69-3f7d58d64ef7</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.store-key</defaultValue>
      <description></description>
      <id>8740a078-d6f6-4a60-990f-d223a22f739f</id>
      <masked>false</masked>
      <name>store-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-id</defaultValue>
      <description></description>
      <id>3788a771-629b-42d5-a013-29d57159b366</id>
      <masked>false</masked>
      <name>order-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-version</defaultValue>
      <description></description>
      <id>5e319803-1cd2-455b-b0a3-75e3ad3be4d2</id>
      <masked>false</masked>
      <name>order-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.payment-id</defaultValue>
      <description></description>
      <id>c6442fd6-ed7e-41a3-b4e4-1f50a73ce6b3</id>
      <masked>false</masked>
      <name>payment-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
