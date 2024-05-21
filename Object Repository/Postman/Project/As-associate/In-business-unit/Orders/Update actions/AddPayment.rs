<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddPayment</name>
   <tag></tag>
   <elementGuidId>346d4d77-b7c5-4c69-9041-d10927610c81</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${order-version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;addPayment\&quot;,\n            \&quot;payment\&quot; : {\n              \&quot;typeId\&quot; : \&quot;payment\&quot;,\n              \&quot;id\&quot; : \&quot;${payment-id}\&quot;\n            }\n          }\n    ]\n}&quot;,
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
      <webElementGuid>72e1b49e-5261-44eb-b646-a9e5d76164df</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>939d0ab7-0059-4b62-a33c-f8c3d57241f5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/as-associate/${associate-id}/in-business-unit/key=${business-unit-key}/orders/${order-id}</restUrl>
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
      <id>c6a860b2-bb7d-4c13-851e-d53ee80111ce</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>d7364980-8589-4842-bb08-63912ae5da44</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.associate-id</defaultValue>
      <description></description>
      <id>20d749a7-57e2-4de2-910a-6f055682d094</id>
      <masked>false</masked>
      <name>associate-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.business-unit-key</defaultValue>
      <description></description>
      <id>71fd12f1-67ac-4ac0-a3ae-54ff96041eba</id>
      <masked>false</masked>
      <name>business-unit-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-id</defaultValue>
      <description></description>
      <id>764fbecd-1ffe-4b78-bad0-87bb50b768c3</id>
      <masked>false</masked>
      <name>order-id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.order-version</defaultValue>
      <description></description>
      <id>ec713e39-03b4-490b-a67d-889f54f68dde</id>
      <masked>false</masked>
      <name>order-version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.payment-id</defaultValue>
      <description></description>
      <id>1b50e28b-047e-4a28-968a-952e3a3f4cdf</id>
      <masked>false</masked>
      <name>payment-id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
